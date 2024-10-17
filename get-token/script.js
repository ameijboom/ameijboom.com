document.addEventListener('musickitloaded', async function () {
    // MusicKit instance is available
    const music = MusicKit.getInstance();
    await music.authorize();

    const { data: result } = await music.api.music('v1/me/library/albums');
    // User's iCloud Music Library Albums
    console.log(result.data);
});
