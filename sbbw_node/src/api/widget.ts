
type SbbwWidgetVectorParam = {
    /**
     * Position or Size in X axis
     */
    x: number | string,
    /**
     * Position or Size in Y axis
     */
    y: number | string,
}

type SbbwWidgetInfo = {
    /**
     * Name of widget
     */
    name: string,
    /**
     * Custom arguments sended by cli when start or close
     */
    widget_args: Array<string>,
}

const getWidgetInfo = (): Promise<SbbwWidgetInfo> =>
    window.rpc.call("widget.info", [])

const movePositionWidget = (param: SbbwWidgetVectorParam): Promise<SbbwWidgetInfo> =>
    window.rpc.call("widget.move", [param.x.toString(), param.y.toString()])

const resizeWidget = (param: SbbwWidgetVectorParam): Promise<SbbwWidgetInfo> =>
    window.rpc.call("widget.resize", [param.x.toString(), param.y.toString()])

export type { SbbwWidgetInfo, SbbwWidgetVectorParam }
export { getWidgetInfo, movePositionWidget, resizeWidget}
