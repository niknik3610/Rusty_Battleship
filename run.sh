docker build -t rusty_battleship .
docker run -p 8000:8000 --rm --name battleship rusty_battleship
