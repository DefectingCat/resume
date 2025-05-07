export function name() {
    return "xfy";
}

export function download(callback) {
    const script = document.createElement("script");
    script.src = "/public/js/html2canvas-pro.min.js";
    script.addEventListener("load", () => {
        console.log("html2canvas loaded");
        html2canvas(document.querySelector("#content")).then(function (canvas) {
            document.body.appendChild(canvas);
        });
    });
    document.body.appendChild(script);
}
