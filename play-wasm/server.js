const fs = require("fs");
const path = require("path");

const server = Bun.serve({
  port: 3000,
  fetch(req) {
    const url = new URL(req.url);
    const filePath = path.join(__dirname, url.pathname === "/" ? "index.html" : url.pathname);

    try {
      if (fs.existsSync(filePath)) {
        const content = fs.readFileSync(filePath);

        const extname = path.extname(filePath);
        let contentType = "text/html";
        if (extname === ".js") contentType = "text/javascript";
        else if (extname === ".wasm") contentType = "application/wasm";
        else if (extname === ".css") contentType = "text/css";

        return new Response(content, {
          headers: { "Content-Type": contentType }
        });
      } else {
        return new Response("File not found", { status: 404 });
      }
    } catch (error) {
      return new Response("Internal Server Error", { status: 500 });
    }
  }
});

console.log(`Server running at http://localhost:${server.port}/`);
