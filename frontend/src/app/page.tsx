'use client';

import type { Message } from "@bindings/Message";
import { useEffect, useRef, useState } from "react";

export default function Home() {
  const [msgs, setMsgs] = useState<Message[]>([]);
	const esRef = useRef<EventSource | null>(null);

	useEffect(() => {
		const es = new EventSource('/hello');
		esRef.current = es;
    es.addEventListener('message', (evt) => {
			try {
				const parsed: Message = JSON.parse(evt.data);
				setMsgs((prev) => [...prev, parsed]);
			} catch {
				console.log(`Error parsing message json: ${evt.data}`);
			}
    });
	  return () => {
      es.close();
      esRef.current = null;
    };
	}, []);
	return (
		<>
			<div className="font-sans">Axum and Next.js Template Page</div>
			<ul>
				{msgs.map((m, i) => (
					<li key={i}>{m.data}</li>
				))}
			</ul>
		</>
	);
}
