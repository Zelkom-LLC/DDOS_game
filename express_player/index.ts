import express, { type Request, type Response } from "express";
import { burnCPU, fibonacciIter, fibonacciRec } from "./strategy";

const app = express();
const port = 8080;

app.get("/", (req, res) => {
    res.send("Hello from Express!");
});

// Health check
app.get("/health", (_: Request, res: Response) => {
    res.sendStatus(200);
});

// Readiness check
app.get("/ready", (_: Request, res: Response) => {
    res.send("Ready!");
});

// Fibonacci recursive handler
app.get("/fib_rec/:iter", (req: Request, res: Response) => {
    const iter = parseInt(req.params.iter!, 10);
    if (isNaN(iter) || iter < 0) {
        return res.status(400).json({ error: "invalid number" });
    }
    const result = fibonacciRec(iter);
    console.log("Defense - ", result);
    res.json({ result });
});

// Fibonacci iterative handler
app.get("/fib_iter/:iter", (req: Request, res: Response) => {
    const iter = parseInt(req.params.iter!, 10);
    if (isNaN(iter) || iter < 0) {
        return res.status(400).json({ error: "invalid number" });
    }
    const result = fibonacciIter(iter);
    console.log("Defense - ", result);
    res.json({ result });
});

// CPU burn handler
app.get("/burn/:iter", (req: Request, res: Response) => {
    const iter = parseInt(req.params.iter!, 10);
    if (isNaN(iter) || iter < 0) {
        return res.status(400).json({ error: "invalid number" });
    }
    const result = burnCPU(iter);
    console.log("Defense - ", result);
    res.json({ result });
});

// Start server
app.listen(port, () => {
    console.log(`Listening on port ${port}...`);
});