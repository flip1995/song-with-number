import pandas as pd
import spotipy
from spotipy.oauth2 import SpotifyClientCredentials
import pprint
from lyricsgenius import Genius
from tqdm import tqdm

cid = ""  # TODO: Spotify client ID
secret = ""  # TODO: Spotify client secret
genius_token = ""  # TODO: Genius API access token
playlist_uri = ""  # TODO: Playlist URI

client_credentials_manager = SpotifyClientCredentials(
    client_id=cid, client_secret=secret
)
sp = spotipy.Spotify(client_credentials_manager=client_credentials_manager)
genius = Genius(genius_token, verbose=False)

pp = pprint.PrettyPrinter(indent=2)

# insert the URI as a string into the function
def get_playlist_tracks(uri_info):
    track = []
    artist = []

    items = [None]
    offset = 0
    while len(items) != 0:
        playlist = sp.playlist_tracks(uri_info, offset=offset)
        items = playlist["items"]

        for x in items:
            x = x["track"]
            track.append(x["name"])
            artist.append(x["artists"][0]["name"])

        offset += len(items)

    df = pd.DataFrame(
        {
            "track": track,
            "artist": artist,
        }
    )

    return df


spotify_data = get_playlist_tracks(playlist_uri)
print(spotify_data)

lyrics = [""] * len(spotify_data)
for i, (track, artist) in tqdm(
    enumerate(zip(spotify_data["track"], spotify_data["artist"]))
):
    try:
        song = genius.search_song(track, artist, get_full_info=False)
        lyrics[i] = song.lyrics
    except:
        print(f'Skipping "{track}" by {artist}')

for i, x in tqdm(enumerate(lyrics)):
    if x == "":
        track = spotify_data["track"][i]
        artist = spotify_data["artist"][i]
        try:
            song = genius.search_song(track, artist, get_full_info=False)
            lyrics[i] = song.lyrics
        except:
            print(f'Giving up on "{track}" by {artist}')

spotify_data.insert(2, "lyrics", lyrics)

spotify_data.to_json("lyrics.json", orient="records", indent=2)
