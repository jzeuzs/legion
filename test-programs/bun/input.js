for await (const chunk of Bun.stdin.stream()) {
	const chunkText = Buffer.from(chunk).toString();
	console.log(`Chunk: ${chunkText}`);
}
