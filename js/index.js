let gen = undefined;

const DEFAULT_SEED = (new Date()).getTime();

function init_background(id, seed) {
    if(!gen) {
        console.error("WASM is not yet read");
        return;
    }
    const w = document.body.offsetWidth;
    const h = document.body.offsetHeight;
    let chunk = gen.gen_background(id || "factorio", w, h, seed !== undefined && seed !== null ? seed : DEFAULT_SEED);
    if(chunk === undefined) return;
    let blob = new Blob([chunk], {type: "image/bmp"});
    let url = window.URL.createObjectURL(blob);
    document.body.style.backgroundImage = "url('"+url+"')";
}

import("../pkg/index.js").catch(console.error).then(m => {
    gen = m;
    init_background();
    let in_progress = 0;
    window.addEventListener('resize', () => {
        in_progress += 1;
        if(in_progress == 1) window.requestAnimationFrame(() => {
            init_background()
            in_progress = 0;
        });
    });
    window.init_background = init_background;
});
