import { Writer as JominiWriter } from 'jomini';
import { Option } from './Option';

export class WriterHelper {
  constructor(private _writer: JominiWriter) {}

  public var(value: string): void {
    this._writer.write_unquoted(value);
  }

  public integer(value: number): void {
    this._writer.write_integer(value);
  }

  public objectStart(): void {
    this._writer.write_object_start();
  }

  public objectEnd(): void {
    this._writer.write_end();
  }

  public arrayStart(): void {
    this._writer.write_array_start();
  }

  public arrayEnd(): void {
    this._writer.write_end();
  }

  public optionalAssignment(lhs: string, rhs: Option<string>): void;
  public optionalAssignment(lhs: string, rhs: Option<number>): void;
  public optionalAssignment(lhs: string, rhs: Option<string | number>): void;
  public optionalAssignment(lhs: string, rhs: Option<string | number>): void {
    rhs.match({
      None: () => {},
      Some: (a) => {
        this.assignment(lhs, a);
      },
    });
  }

  public assignment(lhs: string, rhs: string): void;
  public assignment(lhs: string, rhs: number): void;
  public assignment(lhs: string, rhs: () => void): void;
  public assignment(lhs: string, rhs: string | number | (() => void)): void;
  public assignment(lhs: string, rhs: string | number | (() => void)): void {
    if (typeof rhs === 'string') {
      this.var(lhs);
      this._writer.write_quoted(rhs);
      return;
    }

    if (typeof rhs === 'number') {
      this.var(lhs);
      this.integer(rhs);
      return;
    }

    if (typeof rhs === 'function') {
      this.var(lhs);
      rhs();
      return;
    }
  }

  public object(inner: () => void): void {
    this.objectStart();
    inner();
    this.objectEnd();
  }

  public array(inner: () => void): void {
    this.arrayStart();
    inner();
    this.arrayEnd();
  }
}