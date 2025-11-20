/**
 * Text Streaming Utilities
 *
 * Provides character-by-character text animation for a streaming effect
 */

export interface StreamingTextOptions {
	text: string;
	onChunk?: (chunk: string) => void;
	onComplete?: () => void;
	chunkSize?: number;
	delayMs?: number;
}

/**
 * Stream text character-by-character with a callback
 */
export async function streamText(options: StreamingTextOptions): Promise<void> {
	const { text, onChunk, onComplete, chunkSize = 1, delayMs = 20 } = options;

	let currentIndex = 0;

	while (currentIndex < text.length) {
		const chunk = text.slice(currentIndex, currentIndex + chunkSize);
		if (onChunk) {
			onChunk(chunk);
		}

		currentIndex += chunkSize;

		if (currentIndex < text.length) {
			await new Promise((resolve) => setTimeout(resolve, delayMs));
		}
	}

	if (onComplete) {
		onComplete();
	}
}

/**
 * Svelte store-based streaming text
 */
export function createStreamingText(text: string, speed: 'slow' | 'normal' | 'fast' = 'normal') {
	let currentText = '';
	let subscribers: Array<(text: string) => void> = [];

	const speeds = {
		slow: 50,
		normal: 20,
		fast: 10
	};

	const delayMs = speeds[speed];

	// Start streaming
	let currentIndex = 0;
	const interval = setInterval(() => {
		if (currentIndex < text.length) {
			currentText += text[currentIndex];
			subscribers.forEach((cb) => cb(currentText));
			currentIndex++;
		} else {
			clearInterval(interval);
		}
	}, delayMs);

	return {
		subscribe(callback: (text: string) => void) {
			subscribers.push(callback);
			callback(currentText); // Immediately call with current value
			return () => {
				subscribers = subscribers.filter((cb) => cb !== callback);
			};
		},
		stop() {
			clearInterval(interval);
			currentText = text;
			subscribers.forEach((cb) => cb(currentText));
		}
	};
}

/**
 * Simulate streaming from a complete response
 */
export function simulateStreaming(
	fullText: string,
	onUpdate: (partialText: string) => void,
	onComplete?: () => void,
	speed: 'slow' | 'normal' | 'fast' = 'normal'
): () => void {
	const speeds = {
		slow: 50,
		normal: 15,
		fast: 5
	};

	const delayMs = speeds[speed];
	let currentIndex = 0;
	let cancelled = false;

	const stream = async () => {
		while (currentIndex < fullText.length && !cancelled) {
			onUpdate(fullText.slice(0, currentIndex + 1));
			currentIndex++;
			await new Promise((resolve) => setTimeout(resolve, delayMs));
		}

		if (!cancelled && onComplete) {
			onComplete();
		}
	};

	stream();

	// Return cancellation function
	return () => {
		cancelled = true;
		onUpdate(fullText); // Show full text immediately
		if (onComplete) onComplete();
	};
}
