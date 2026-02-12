import { useState, useCallback } from 'react'
import './App.css'

const uid = () => Math.random().toString(16).slice(2) + Date.now().toString(16)

function App() {
  const [gallery, setGallery] = useState([])
  const [albums, setAlbums] = useState([])
  const [search, setSearch] = useState('')
  const [modalOpen, setModalOpen] = useState(false)
  const [albumName, setAlbumName] = useState('')
  const [error, setError] = useState('')
  const [uploading, setUploading] = useState(false)
  const [progress, setProgress] = useState({ current: 0, total: 0, name: '' })

  const filteredGallery = search.trim()
    ? gallery.filter(x => x.name.toLowerCase().includes(search.toLowerCase()) ||
      x.tags.some(t => t.toLowerCase().includes(search.toLowerCase())))
    : gallery

  const selectedCount = gallery.filter(x => x.selected).length

  const addFiles = useCallback((files) => {
    const newItems = [...files]
      .filter(f => f.type.startsWith('image/'))
      .map(f => ({
        id: uid(),
        file: f,
        name: f.name,
        url: URL.createObjectURL(f),
        tags: [],
        selected: false,
        shareLink: null
      }))
    setGallery(prev => [...prev, ...newItems])
  }, [])

  const handleDrop = useCallback((e) => {
    e.preventDefault()
    e.currentTarget.classList.remove('dragover')
    if (e.dataTransfer.files.length) addFiles(e.dataTransfer.files)
  }, [addFiles])

  const handleDragOver = (e) => {
    e.preventDefault()
    e.currentTarget.classList.add('dragover')
  }

  const handleDragLeave = (e) => {
    e.currentTarget.classList.remove('dragover')
  }

  const deleteImg = (id) => {
    setGallery(prev => {
      const item = prev.find(x => x.id === id)
      if (item) URL.revokeObjectURL(item.url)
      return prev.filter(x => x.id !== id)
    })
  }

  const moveImg = (idx, dir) => {
    const newIdx = idx + dir
    if (newIdx >= 0 && newIdx < gallery.length) {
      setGallery(prev => {
        const arr = [...prev];
        [arr[idx], arr[newIdx]] = [arr[newIdx], arr[idx]]
        return arr
      })
    }
  }

  const toggleSelect = (id) => {
    setGallery(prev => prev.map(x => x.id === id ? { ...x, selected: !x.selected } : x))
  }

  const selectAll = () => setGallery(prev => prev.map(x => ({ ...x, selected: true })))
  const deselectAll = () => setGallery(prev => prev.map(x => ({ ...x, selected: false })))

  const addTag = (id, raw) => {
    const tags = raw.split(',').map(t => t.trim()).filter(Boolean)
    setGallery(prev => prev.map(x => {
      if (x.id === id) {
        const newTags = [...x.tags]
        tags.forEach(t => { if (!newTags.includes(t)) newTags.push(t) })
        return { ...x, tags: newTags }
      }
      return x
    }))
  }

  const removeTag = (id, tag) => {
    setGallery(prev => prev.map(x => x.id === id ? { ...x, tags: x.tags.filter(t => t !== tag) } : x))
  }

  const clearAll = () => {
    gallery.forEach(x => URL.revokeObjectURL(x.url))
    setGallery([])
  }

  const createAlbum = () => {
    const selected = gallery.filter(x => x.selected)
    if (!selected.length || !albumName.trim()) return
    setAlbums(prev => [...prev, { id: uid(), name: albumName.trim(), ids: selected.map(x => x.id) }])
    deselectAll()
    setModalOpen(false)
    setAlbumName('')
  }

  const deleteAlbum = (id) => {
    setAlbums(prev => prev.filter(a => a.id !== id))
  }

  const copyLink = async (link) => {
    await navigator.clipboard.writeText(link)
  }

  const uploadToServer = async () => {
    setError('')
    if (!gallery.length) {
      setError('Add images first')
      return
    }

    const toUpload = gallery.filter(x => !x.shareLink)
    if (!toUpload.length) {
      setError('All images already uploaded!')
      return
    }

    setUploading(true)
    try {
      // Step 1: Create a new gallery
      const createRes = await fetch('/api/gallery', { method: 'POST' })
      if (!createRes.ok) throw new Error(await createRes.text())
      const { gallery_id } = await createRes.json()

      // Step 2: Upload images to the gallery
      const fd = new FormData()
      toUpload.forEach(x => fd.append('image', x.file))

      const uploadRes = await fetch(`/api/gallery/${gallery_id}/upload`, { method: 'POST', body: fd })
      if (!uploadRes.ok) throw new Error(await uploadRes.text())
      const data = await uploadRes.json()

      if (data.images) {
        // Map uploaded URLs back to the items that were uploaded
        const uploadedIds = toUpload.map(x => x.id)
        setGallery(prev => prev.map(x => {
          const uploadIdx = uploadedIds.indexOf(x.id)
          if (uploadIdx !== -1 && data.images[uploadIdx]) {
            return { ...x, shareLink: data.images[uploadIdx] }
          }
          return x
        }))
      }
    } catch (e) {
      setError(e.message)
    } finally {
      setUploading(false)
    }
  }

  return (
    <div className="app">
      <header className="header">
        <a href="#" className="logo">
          <span className="icon">photo_library</span>
          <h1>Kioku</h1>
        </a>
        <div className="search-box">
          <span className="icon">search</span>
          <input
            type="text"
            placeholder="Search photos..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
        <label className="header-btn">
          <span className="icon">cloud_upload</span>Upload
          <input
            type="file"
            accept="image/*"
            multiple
            hidden
            onChange={(e) => { if (e.target.files.length) { addFiles(e.target.files); e.target.value = '' } }}
          />
        </label>
      </header>

      <main className="main">
        <div className="toolbar">
          <div className="toolbar-left">
            <h1 className="page-title">Photos</h1>
            <span className="photo-count">{filteredGallery.length} photo{filteredGallery.length !== 1 ? 's' : ''}</span>
          </div>
          <div className="toolbar-actions">
            <button className="btn primary" onClick={uploadToServer} disabled={uploading}>
              <span className="icon">backup</span>{uploading ? 'Uploading...' : 'Upload to Server'}
            </button>
            <button className="btn danger" onClick={clearAll}>
              <span className="icon">delete_sweep</span>Clear All
            </button>
          </div>
        </div>

        {!gallery.length && (
          <div
            className="upload-zone"
            onDrop={handleDrop}
            onDragOver={handleDragOver}
            onDragLeave={handleDragLeave}
          >
            <span className="icon">add_photo_alternate</span>
            <h3>Upload your photos</h3>
            <p>Drag and drop images here or click to browse</p>
            <label className="btn">
              Browse Files
              <input
                type="file"
                accept="image/*"
                multiple
                hidden
                onChange={(e) => { if (e.target.files.length) { addFiles(e.target.files); e.target.value = '' } }}
              />
            </label>
          </div>
        )}

        {error && <div className="error show">{error}</div>}

        {uploading && (
          <div className="upload-progress show">
            <span>Uploading {progress.current}/{progress.total}: {progress.name}</span>
            <div className="progress-bar">
              <div className="progress-fill" style={{ width: `${(progress.current / progress.total) * 100}%` }} />
            </div>
          </div>
        )}

        <div className="masonry">
          {filteredGallery.map((item, idx) => (
            <PhotoCard
              key={item.id}
              item={item}
              idx={gallery.indexOf(item)}
              hasSel={selectedCount > 0}
              onToggle={() => toggleSelect(item.id)}
              onDelete={() => deleteImg(item.id)}
              onMove={(dir) => moveImg(gallery.indexOf(item), dir)}
              onAddTag={(tag) => addTag(item.id, tag)}
              onRemoveTag={(tag) => removeTag(item.id, tag)}
              onCopy={copyLink}
            />
          ))}
        </div>

        {!filteredGallery.length && gallery.length === 0 && (
          <div className="empty">
            <span className="icon">image_search</span>
            <h3>No photos yet</h3>
            <p>Upload some images to get started</p>
          </div>
        )}

        {albums.length > 0 && (
          <div className="albums">
            <h2 className="section-title">Collections</h2>
            <div className="album-grid">
              {albums.map(album => (
                <AlbumCard
                  key={album.id}
                  album={album}
                  gallery={gallery}
                  onDelete={() => deleteAlbum(album.id)}
                />
              ))}
            </div>
          </div>
        )}
      </main>

      {selectedCount > 0 && (
        <div className="sel-bar">
          <span className="sel-count">{selectedCount} selected</span>
          <div className="divider" />
          <button className="sel-btn" onClick={selectAll}>
            <span className="icon">select_all</span>Select All
          </button>
          <button className="sel-btn" onClick={deselectAll}>
            <span className="icon">deselect</span>Clear
          </button>
          <div className="divider" />
          <button className="sel-btn primary" onClick={() => setModalOpen(true)}>
            <span className="icon">create_new_folder</span>Create Album
          </button>
        </div>
      )}

      {modalOpen && (
        <div className="modal-bg" onClick={(e) => e.target === e.currentTarget && setModalOpen(false)}>
          <div className="modal">
            <h3>New Album</h3>
            <input
              type="text"
              className="modal-input"
              placeholder="Album name..."
              value={albumName}
              onChange={(e) => setAlbumName(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && createAlbum()}
              autoFocus
            />
            <div className="modal-actions">
              <button className="btn" onClick={() => setModalOpen(false)}>Cancel</button>
              <button className="btn primary" onClick={createAlbum}>Create</button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

function PhotoCard({ item, idx, hasSel, onToggle, onDelete, onMove, onAddTag, onRemoveTag, onCopy }) {
  const [tagInput, setTagInput] = useState('')

  const handleAddTag = () => {
    if (tagInput.trim()) {
      onAddTag(tagInput)
      setTagInput('')
    }
  }

  return (
    <div className={`photo${item.selected ? ' selected' : ''}`}>
      <img src={item.url} alt={item.name} loading="lazy" />
      <div className="photo-overlay">
        <div className="card-top">
          <div
            className={`checkbox${item.selected ? ' checked visible' : ''}${hasSel ? ' visible' : ''}`}
            onClick={onToggle}
          >
            {item.selected && <span className="icon">check</span>}
          </div>
          <div className="card-btns">
            <button className="card-btn" onClick={() => onMove(-1)} title="Move left">
              <span className="icon">chevron_left</span>
            </button>
            <button className="card-btn" onClick={() => onMove(1)} title="Move right">
              <span className="icon">chevron_right</span>
            </button>
            <button className="card-btn del" onClick={onDelete} title="Delete">
              <span className="icon">delete</span>
            </button>
          </div>
        </div>
        <div className="card-bottom">
          <div className="photo-name">{item.name}</div>
          <div className="tag-row">
            <input
              className="tag-input"
              placeholder="Add tags..."
              value={tagInput}
              onChange={(e) => setTagInput(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && handleAddTag()}
            />
            <button className="tag-add" onClick={handleAddTag}>Add</button>
          </div>
          {item.tags.length > 0 && (
            <div className="tags">
              {item.tags.map(tag => (
                <span key={tag} className="tag">
                  {tag}
                  <button className="tag-x" onClick={() => onRemoveTag(tag)}>×</button>
                </span>
              ))}
            </div>
          )}
          {item.shareLink && (
            <div className="share-link">
              <a href={item.shareLink} target="_blank" rel="noopener noreferrer">{item.shareLink}</a>
              <button className="copy-btn" onClick={() => onCopy(item.shareLink)} title="Copy link">
                <span className="icon">content_copy</span>
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

function AlbumCard({ album, gallery, onDelete }) {
  const images = album.ids.map(id => gallery.find(x => x.id === id)).filter(Boolean).slice(0, 4)

  return (
    <div className="album-card">
      <div className="album-cover">
        {[0, 1, 2, 3].map(i => (
          images[i]
            ? <img key={i} src={images[i].url} loading="lazy" />
            : <div key={i} className="ph"><span className="icon">image</span></div>
        ))}
      </div>
      <div className="album-info">
        <div className="album-name">{album.name}</div>
        <div className="album-count">{album.ids.filter(id => gallery.find(x => x.id === id)).length} photos</div>
      </div>
      <div className="album-actions">
        <button className="btn" onClick={onDelete}>
          <span className="icon">delete</span>Remove
        </button>
      </div>
    </div>
  )
}

export default App
