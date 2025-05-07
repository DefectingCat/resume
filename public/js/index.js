export function name() {
    return "xfy";
}

export async function download(callback) {
    return new Promise((resolve, reject) => {
        const script = document.createElement("script");
        script.src = "/public/js/html2canvas-pro.min.js";
        script.addEventListener("load", async () => {
            console.log("html2canvas loaded");
            const canvas = await html2canvas(
                document.querySelector("#content"),
            );
            document.body.appendChild(canvas);
            resolve();
        });
        script.addEventListener("error", reject);
        document.body.appendChild(script);
    });
}
