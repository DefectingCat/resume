export function name() {
    return "xfy";
}

export async function download() {
    return new Promise((resolve, reject) => {
        const script = document.createElement("script");
        script.src = "/public/js/html2canvas-pro.min.js";
        script.addEventListener("load", async () => {
            console.log("html2canvas loaded");
            const content = document.querySelector("#content");
            if (!content) {
                reject("content not found");
            }
            const canvas = await html2canvas(content);
            const imgData = canvas.toDataURL("image/png");
            await generatePdf(canvas, imgData);
            script.remove();
            resolve();
        });
        script.addEventListener("error", reject);
        document.body.appendChild(script);
    });
}

async function generatePdf(canvas, imgData) {
    return new Promise((resolve, reject) => {
        const script = document.createElement("script");
        script.src = "/public/js/jspdf.umd.min.js";
        script.addEventListener("load", async () => {
            console.log("jspdf loaded");
            const pdf = new jspdf.jsPDF();
            const imgWidth = pdf.internal.pageSize.getWidth();
            const imgHeight = (canvas.height * imgWidth) / canvas.width;
            pdf.addImage(imgData, "PNG", 0, 0, imgWidth, imgHeight);
            pdf.save("resume-lilinjun.pdf");
            script.remove();
            resolve();
        });
        script.addEventListener("error", reject);
        document.body.appendChild(script);
    });
}
