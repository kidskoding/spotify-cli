# spotify-cli
A Command Line Interface (CLI) for Spotify built using Rust, where users can manage their Spotify songs and playlists through the Command Line

# Enable command globally
### Windows: 
<ol>
  <li>Open Command Prompt as an Administrator</li>
  <li><code>cd ~/Downloads/spotify-cli</code></li>
  <li><code>mkdir "C:\Program Files\spotify"</code></li>
  <li><code>copy target\release\spotify.exe "C:\Program Files\spotify\"</code></li>
  <li>Add to System Environment PATH Variables</li>
  <ul>
    <li>Go to system variables and edit PATH and add the path <code>C:\Program Files\spotify\</code></li>
  </ul>
</ol>

### UNIX Systems (Mac and Linux): 
<ol>
  <li><code>cd ~/Downloads/spotify-cli</code></li>
  <li><code>sudo cp target/release/spotify usr/local/bin</code></li>
</ol>
