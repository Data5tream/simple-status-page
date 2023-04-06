<footer>
	<div class="content">
		&copy; {new Date().getFullYear()}, Simon Stefan Barth | <a href='https://github.com/Data5tream' target='_blank' rel='noreferrer'>Github</a>
	</div>
</footer>

<style lang="scss">
	footer {
		width: 100%;
		padding: 16px 0 8px;
		background: var(--c-black);
		color: var(--c-white);
	}

	.content {
		max-width: 1400px;
		margin: 0 auto;
		font-family: monospace;
		font-size: 16px;
	}

	a {
		&,
		&:visited {
			color: var(--c-white);
		}

		&:hover,
		&:active {
			background: var(--c-white);
			color: var(--c-black);
		}
	}
</style>
