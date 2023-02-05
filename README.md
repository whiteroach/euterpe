## Euterpe

Euterpe is an API that serves the imaginary Euterpe music platform.

**Usage:**

```
Cargo run
```

Or run it with Docker:

```
docker build -t euterpe .
docker run -p 8000:8000 -h euterpe
```

**Curiosity:**

> Euterpe was one of the Muses in Greek mythology, presiding over music.</br> - [Wikipedia](https://en.wikipedia.org/wiki/Euterpe)

**Todo:** </br>

- [ ] Add migrations for:
- - [x] Labels
- - [x] Albums
- - [ ] Bands
- - [ ] band_artist
- - [ ] band_label
- - [ ] artist_label
- - [ ] user_picture
- - [ ] album_picture
- - [ ] album_track_list
- - [ ] playlist_track_list
        <br>

- [ ] Generate entities
- [ ] Add docker-compose
