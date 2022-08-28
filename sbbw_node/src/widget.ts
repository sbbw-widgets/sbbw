import { invoke } from "./sbbw"

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
    invoke("widget.info", null)

const movePositionWidget = (param: SbbwWidgetVectorParam): Promise<SbbwWidgetInfo> =>
    invoke("widget.move", param)

const resizeWidget = (param: SbbwWidgetVectorParam): Promise<SbbwWidgetInfo> =>
    invoke("widget.resize", param)

export type { SbbwWidgetInfo, SbbwWidgetVectorParam }
export { getWidgetInfo, movePositionWidget, resizeWidget}
