import * as signalR from "@microsoft/signalr";
import { HubConnectionState } from "@microsoft/signalr";
import { useEffect } from "react";

import { useActivity } from "./activity/hooks/activity.hook";
import { useActivityRealtime } from "./activity/realtime/useActivityRealtime";
import { useChatRealtime } from "./chat/realtime/useChatRealtime";
import { useConversationRealtime } from "./friends/realtime/useConversationRealtime";
import { useRelationshipRealtime } from "./friends/realtime/useRelationshipRealtime";
import { queryClient } from "./queryClient";
import callApi from "./tokens/callApi";
import i18n from "./translation/i18n";

let hub: signalR.HubConnection | null = null;

let lifecycle: Promise<void> = Promise.resolve();

function enqueue(step: () => Promise<void>): void {
	lifecycle = lifecycle.then(step).catch(console.error);
}

function stopHub(connection: signalR.HubConnection): void {
	enqueue(async (): Promise<void> => {
		await connection.stop();
	});
}

async function stopIfLoggedOut(
	connection: signalR.HubConnection,
): Promise<void> {
	const res = await callApi("/api/me");
	if (res.status === 401) {
		stopHub(connection);
	}
}

function onConnected(): void {
	const activity = useActivity.getState();
	if (!activity.selfId) {
		return;
	}
	activity.reportActivity();
	activity.askOthersActivity();
}

export function getConnection(): signalR.HubConnection {
	if (!hub) {
		const connection = new signalR.HubConnectionBuilder()
			.withUrl("/api/", {})
			.withAutomaticReconnect()
			.build();
		connection.onreconnecting((): void => {
			stopIfLoggedOut(connection).catch(console.error);
		});
		connection.onreconnected((): void => {
			void queryClient.invalidateQueries();
			onConnected();
		});
		hub = connection;
	}
	return hub;
}

export async function hubSend(
	method: string,
	...args: unknown[]
): Promise<void> {
	const connection = getConnection();
	if (connection.state !== HubConnectionState.Connected) {
		return;
	}
	try {
		await connection.send(method, ...args);
	} catch (error) {
		console.warn(`Realtime ${method} not sent:`, error);
	}
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export async function hubInvoke<T = any>(
	method: string,
	...args: unknown[]
): Promise<T> {
	const connection = getConnection();
	if (connection.state !== HubConnectionState.Connected) {
		throw new Error(i18n.t("apiErrors.codes.NOT_AUTHENTICATED"));
	}
	return connection.invoke(method, ...args);
}

export function useRealtime(): void {
	useEffect(() => {
		const connection = getConnection();
		enqueue(async (): Promise<void> => {
			if (connection.state === HubConnectionState.Disconnected) {
				await connection.start();
				onConnected();
			}
		});

		return (): void => {
			stopHub(connection);
			void queryClient.invalidateQueries({ refetchType: "none" });
		};
	}, []);

	useConversationRealtime();
	useRelationshipRealtime();
	useChatRealtime();
	useActivityRealtime();
}
