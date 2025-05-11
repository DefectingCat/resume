export function name() {
    return "xfy";
}

/**
 * Use html2canvas and jspdf to generate pdf
 */
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
            // document.body.append(canvas);
            const imgData = canvas.toDataURL("image/png");
            await generatePdf(canvas, imgData);
            script.remove();
            resolve();
        });
        script.addEventListener("error", reject);
        document.body.appendChild(script);
    });
}

/**
 *  Use jspdf to generate canvas to pdf
 */
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
