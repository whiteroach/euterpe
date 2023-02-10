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

- [x] Add migrations for:
- - [x] Labels
- - [x] Albums
- - [x] Bands
- - [x] band_user
- - [x] band_label
- - [x] user_label
- - [x] band_picture
- - [x] user_picture
- - [x] album_picture
- - [x] album_track
- - [x] playlist_track_list
        <br>

- [x] Generate entities
- [ ] Set many-to-many relationships [sea-orm doc](https://www.sea-ql.org/SeaORM/docs/relation/many-to-many/)
- [ ] Add docker-compose
