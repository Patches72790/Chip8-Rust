import express from "express";
import path from "path";

const app = express();

app.use(express.static("public"));
app.use(express.urlencoded({  extended: true}))

app.get("/", (req: express.Request, res: express.Response) => {
  res.sendFile(path.join(__dirname, "public", "index.html"));
});

app.listen("8080", () => {
  console.log("listening on port 8080");
});
