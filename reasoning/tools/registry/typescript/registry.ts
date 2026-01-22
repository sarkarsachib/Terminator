export type ToolSpec = {
  name: string;
  description: string;
};

export class ToolRegistry {
  private tools = new Map<string, ToolSpec>();

  register(spec: ToolSpec): void {
    this.tools.set(spec.name, spec);
  }

  list(): ToolSpec[] {
    return [...this.tools.values()];
  }
}
