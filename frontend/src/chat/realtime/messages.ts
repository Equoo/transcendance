import { hubInvoke } from "@/realtime";

import type { Message } from "../api/chat.api";

export async function sendMessage(
	channelId: string,
	content: string,
	messageReference: string | undefined,
): Promise<Message> {
	return hubInvoke("CreateMessage", channelId, {
		content,
		messageReference,
	});
}

export async function updateMessage(
	channelId: string,
	id: string,
	content: string,
): Promise<void> {
	return hubInvoke("UpdateMessage", channelId, id, {
		content,
	});
}

export async function deleteMessage(
	channelId: string,
	id: string,
): Promise<void> {
	return hubInvoke("DeleteMessage", channelId, id);
}
