export type ExecRequest = {
  language: string;
  source: string;
};

export type ExecResult = {
  stdout: string;
  stderr: string;
  exitCode: number;
};

export async function run(_req: ExecRequest): Promise<ExecResult> {
  return { stdout: "", stderr: "not implemented", exitCode: 1 };
}
