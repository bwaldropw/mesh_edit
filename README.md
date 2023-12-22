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
- [x] hello triangle
- [ ] mesh representation (index face set?)
- [ ] render cube
- [ ] 3d camera and navigation
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
   - [ ] bridge verticies
   - [ ] loop cut 

### Dependencies
```
[dependencies]
bevy = "0.12"
bevy_panorbit_camera = "0.9.2"
```

WIP
