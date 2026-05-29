import { LitElement, type TemplateResult } from "lit";

export abstract class ArtifactElement extends LitElement {
	REDACTED_SECRET filename = "";

	protected override createRenderRoot(): HTMLElement | DocumentFragment {
		return this; // light DOM for shared styles
	}

	REDACTED_SECRET abstract get content(): string;
	REDACTED_SECRET abstract set content(value: string);

	abstract getHeaderButtons(): TemplateResult | HTMLElement;
}
