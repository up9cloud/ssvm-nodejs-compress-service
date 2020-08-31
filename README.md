# ssvm-nodejs-compress-service

Can I [Get a Free Raspberry Pi Kit](https://www.secondstate.io/articles/raspberry-pi-for-free-20200709/)?

## Usage

```bash
# up service
docker run --name 7z -d -p 3000:3000 sstc/ssvm-nodejs-compress-service

# upload a file and download the compressed file
curl localhost:3000 -F 'file=@README.md' -o README.md.7z
```

## Dev

### Up service

> create container

```bash
./up.sh 3001

# or from other image
docker build -f docker/Dockerfile.debug_base -t ssvm-builder .
./up.sh 3001 ssvm-builder
```

> build and run

```bash
ssvmup build
node node/app.js
```

### Test

```bash
curl localhost:3001 -F 'file=@README.md' -o README.md.7z
stat README.md.7z
```
