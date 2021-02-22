import {html, svg, render} from 'https://unpkg.com/lit-html?module'

const colors = ['#444', '#f06', '#06f']

const n = 5
const board = Array(n * n).fill(0)
let turn = 1

function renderBoard(board, onClick) {
	const s = 40
	const n = Math.sqrt(board.length)
	const range = [...Array(n).keys()]
	const style = (i, j) => colors[board[i*n + j]]

	let d = ''
	for (let i = 0; i < 6; i++) {
		const a = 2 * Math.PI / 6 * i + Math.PI / 2
		d += `${i === 0 ? 'M' : 'L'} ${Math.cos(a)*s*0.45},${Math.sin(a)*s*0.45} `
	}
	d += 'Z'

	const w = (1.5 * (n-1) + 2) * s
	const h = ((n-1) + 2) * s
	return svg`<svg
		xmlns="http://www.w3.org/2000/svg"
		viewBox=${`0 0 ${w} ${h}`}
		style=${`width: ${w}px; height: ${h}px;`}
	>${
		range.flatMap(i => range.map(j => svg`
			<path
				transform=${`translate(${(j+i/2+1)*s} ${(i*Math.sqrt(3)/2 + 1)*s})`}
				class="piece"
				fill=${style(i, j)}
				d=${d}
				@click=${() => onClick(i*n+j)}
			/>
		`))
	}</svg>`
}

const boardDiv = document.querySelector('.board')

import init, {Client} from './wasm/pkg/wasm.js'
init().then(() => {
	const client = Client.new(n)

	function rerender() {
		render(renderBoard(board, onClick), boardDiv)
	}

	function play(i) {
		client.play(i)

		board[i] = turn
		turn = turn === 2 ? 1 : 2
		rerender()

		if (client.terminal_value()) {
			alert('Game over!')
		}
	}

	async function onClick(i) {
		play(i)
		playSearch()
	}

	async function playSearch() {
		await new Promise((resolve) => setTimeout(resolve))
		const its = 30000
		const action = client.think(its)
		await new Promise((resolve) => setTimeout(resolve))
		play(action)
	}

	rerender()

	playSearch()
})

