<div class="separator" />

<style lang="scss">
	.separator {
		background-color: variables.$grey-3;
		position: relative;

		&::before {
			content: '';
			position: absolute;
			height: 100%;
			border: none;
			border-right: 0.5rem dashed variables.$grey-1;
		}
	}
</style>
