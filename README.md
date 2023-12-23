## Mesh Edit

### build instructions
1. install rust and cargo, [here](https://rustup.rs/).
2. clone repo
   ```bash
   git clone https://github.com/bwaldropw/mesh_edit.git
   cd mesh_edit
3. build and run
   ```bash
   cargo run

### TODOs
- [ ] mesh representation
- [x] render cube
   - [ ] fix normals
- [ ] dev ui
   - [ ] show fps 
- [x] 3d camera and navigation
   - [ ] keybinds
- [ ] input handling
- [ ] mesh utils
   - [ ] mesh traversal 
- [ ] vertex/face selection
- [ ] gizmo
- [ ] file io
- [ ] ui implementation
- [ ] mesh transformations
   - [ ] extrude face
   - [ ] bridge vertices
   - [ ] loop cut 

### Dependencies
```
[dependencies]
bevy = "0.12"
bevy_panorbit_camera = "0.9.2"
tobj = "4.0.0"
```

WIP
