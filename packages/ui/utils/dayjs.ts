import dayjsImported from "dayjs"
import relativeTime from "dayjs/plugin/relativeTime"

dayjsImported.extend(relativeTime)

export const dayjs = dayjsImported
