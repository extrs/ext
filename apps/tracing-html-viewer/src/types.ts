

export interface TraceEvent {
    fields: {
        message: string
    },

    span?: Span
    spans?: Span[]

    level: string
    target: string
    threadId: string
    threadName: string

    timestamp: string
}

export interface Span {
    name: string

    [field: string]: string
}