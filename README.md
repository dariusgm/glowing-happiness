# glowing-happiness
Analyse the distribution of files and its content

## building
### install rust
```bash
sudo apt update -y && sudo apt upgrade -y
sudo apt install curl build-essential gcc make -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### build project
```bash
cargo build
```

### fetch repositories for integration tests
```
git clone git@github.com:CodeEditApp/CodeEdit.git tests/repositories/CodeEdit && cd tests/repositories/CodeEdit && git checkout 347c28356a870af456189519a41ea1351626787d && rm -rf .git && cd ../../..
git clone git@github.com:helix-editor/helix.git tests/repositories/helix && cd tests/repositories/helix && git checkout e8730ca5fd72e3bb275b4d825de40475eabea174 && rm -rf .git && cd ../../..
```

## usage
### Input
provide the input path of the directory / repository that you want to analyse:
```bash
glowing-happiness --input .
```
## Parallel Execution
The files will be analysed in parallel. To reduce the IO load on your device, you can set
`RAYON_NUM_THREADS=4` to only use 4 threads instead of all.
