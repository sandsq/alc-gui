/**@param {string} str*/
function isDigit(str) {
	const num = parseInt(str, 10);
	return isFinite(num);
}

/**@param {string} str */
export function split_layer_to_rows(str) {
	let rows = str.split("\n")
	rows = rows.filter((x) => x.trim().length > 0)
	rows = rows.filter((x) => x.split(/\s+/).filter((y) => !isDigit(y) && y != "").length > 0)
	return rows
}

/**
 * @param {string} str 
 * */
export function split_row_to_columns(str) {
	// 0|E_10 D_10
	let str_filtered = str.includes("|") ? str.split("|")[1] : str
	let cols = str_filtered.split(/\s+/)
	cols = cols.filter((x) => x != "")
	return cols
}

/**@param {number} delay*/
export const sleep = (delay) => new Promise((resolve) => setTimeout(resolve, delay))