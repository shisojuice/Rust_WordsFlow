import init, { str_to_png } from './rust_wordsflow.js';

document.getElementById("words_input").style.fontFamily = "Noto Sans JP";
async function run() {
    await init();
    document.getElementById("GeneBtn").addEventListener("click", async (event) => {
        const words = document.getElementById("words_input").value;
        if (words.length === 0) {
            window.confirm("err:Wordsをセットしてください");
            return;
        }
        if (words.length > 50) {
            window.confirm("err:50字以下でWordsをセットしてください");
            return;
        }
        await fetchFontAsUint8Array(document.getElementById("words_font").value)
            .then(uint8Array => {
                const ret = str_to_png(words, 512, uint8Array);
                const speedparam = "speed=" + document.getElementById("words_speed").value;
                localStorage.clear();
                localStorage.setItem("img_src", ret);
                window.open("./flow.html?" + speedparam, "_blank", "width=" + screen.width + ",height=" + screen.height);

            });
    });
    document.getElementById("words_font").addEventListener("change", (event) => {
        document.getElementById("words_input").style.fontFamily = event.target.selectedOptions[0].dataset.font;
    })
}
run();


const fetchFontAsUint8Array = async (fontPath) => {
    try {
        const response = await fetch(fontPath);
        const arrayBuffer = await response.arrayBuffer();
        return new Uint8Array(arrayBuffer);
    } catch (error) {
        console.error('fontfile_error:', error);
        return null;
    }
};