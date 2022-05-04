

export type SpanDecls = {
    [id: number]: SpanDecl;
}

export interface TraceEvent {
    metadata: Metadata
    fields: { [key: string]: string }
}

export interface SpanTraceData {
    enteredAt: string | null
    exitedAt: string | null
    closedAt: string | null
    createdAt: string

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