package workflows

docs: _#borsWorkflow & {
	name: "docs"

	on: push: branches: borsBranches

	env: CARGO_TERM_COLOR: "always"

	jobs: {
		changes: _#changes

		markdownFormat: {
			name: "markdown / format"
			needs: ["changes"]
			"runs-on": defaultRunner
			"if":      "${{ needs.changes.outputs.markdown == 'true' }}"
			steps: [
				_#checkoutCode,
				_#prettier & {
					with: prettier_options: """
						--check --color --prose-wrap always ${{ needs.changes.outputs.markdown_files }}
						"""
				},
			]
		}

		bors: needs: [
			"markdownFormat",
		]
	}
}
