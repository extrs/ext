

export type SpanDecls = {
    [id: number]: SpanDecl;
}

export interface TraceEvent {
    metadata: Metadata
    fields: { [key: string]: string }
}

export interface SpanTraceData {
    closedAt: string | null
    enteredAt: string
    events: TraceEvent[]

    spans: [id: number, data: SpanTraceData][]
}

export interface SpanDecl {
    attrs: { [key: string]: string }
    metadata: Metadata
}

export interface Metadata {
    level: string

    name: string
    target: string

    file: string | null
    line: number | null

    modulePath: string | null
}