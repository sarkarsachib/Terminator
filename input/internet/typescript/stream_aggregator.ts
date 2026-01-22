export type RawStreamEvent = {
  topic: string;
  payload: Uint8Array;
};

export function aggregate(events: RawStreamEvent[]): RawStreamEvent[] {
  return events;
}
