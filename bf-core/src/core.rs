// メモリセル
// 一般的bf
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ValueCell(u8);

enum ValueCellOperation {
    ShiftR,
    ShiftL,
    Increment,
    Decrement,
}

impl ValueCell {
    fn value_change(&mut self, instruction: ValueCellOperation) {
        match instruction {
            ValueCellOperation::ShiftR => {
                self.0 >>= 1;
            }
            ValueCellOperation::ShiftL => {
                self.0 <<= 1;
            }
            ValueCellOperation::Increment => {
                self.0 += 1;
            }
            ValueCellOperation::Decrement => {
                self.0 -= 1;
            }
        }
    }
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

// データメモリ
#[derive(Default, Debug)]
pub struct Memory {
    // ポインタの位置を数字で持ちたくないのでこうなる
    before: Vec<ValueCell>, // ポインタの前のメモリセル
    after: Vec<ValueCell>,  // ポインタの後のメモリセル
    pub current: ValueCell, // 現在のポインタの指すメモリセル
}

enum MemoryPointerMove {
    Right,
    Left,
}

impl Memory {
    fn move_pointer(&mut self, instruction: MemoryPointerMove) {
        match instruction {
            MemoryPointerMove::Right => {
                self.before.push(self.current.clone());
                let next_cell = self.after.pop();
                self.current = next_cell.unwrap_or_default();
            }
            MemoryPointerMove::Left => {
                let next_cell = self.before.pop();
                if let Some(next_cell) = next_cell {
                    self.current = next_cell;
                    self.after.push(self.current.clone());
                }
            }
        }
    }
}

// 命令セット
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    PInc,   // ポインタをインクリメント（右に一つずらす）
    PDec,   // ポインタをデクリメント（左に一つずらす）
    VInc,   // データをインクリメント
    VDec,   // データをデクリメント
    Output, // データを出力
    Input,  // データに入力
    Open,   // ポインタの指す値がゼロならCloseの直後まで飛ぶ
    Close,  // 対応するOpenまで戻る
    // 拡張命令
    VShiftR, // データを右にシフト
    VShiftL, // データを左にシフト
}

// 命令セットの列がプログラム
pub struct Program(Vec<Operation>);

// 本当はOpen, Closeの命令に対応するような入れ子構造になったほうやつもあったほうがいい気がするがとりあえずこれで

// 現在のポインタの指す値が与えられたら次の命令は計算できるのでこれで十分
pub struct Analyzer {
    pub program: Program,
    pub index: usize,
}

impl Analyzer {
    pub fn initialize(program: Program) -> Self {
        Self { program, index: 0 }
    }
    // 次の命令を計算するやつ
    fn next(&mut self, cell: ValueCell) -> Option<Operation> {
        // 次の命令を見てみる
        self.index += 1;
        let next_operation = self.program.0.get(self.index);
        match next_operation {
            // 次の命令がないときはNoneを返す
            None => None,
            // 対応するOpenに移動するか、次に進むか
            Some(Operation::Close) => {
                // cellがゼロの場合は次のに進んで良い
                if cell.is_zero() {
                    return self.next(cell);
                }
                let mut close_count = 0;
                // 対応するOpenまで移動する
                while {
                    self.index -= 1;
                    let before_op = self.program.0.get(self.index);
                    let is_end = match before_op {
                        // 本来は不正なbfプログラムなのだが、即座にtrueにするだけで許す
                        None => true,
                        // これがOpenならば対応してるOpenかどうかをチェック
                        Some(Operation::Open) => {
                            if close_count == 0 {
                                true
                            } else {
                                close_count -= 1;
                                false
                            }
                        }
                        // Closeならまだ続きを読む
                        Some(Operation::Close) => {
                            close_count += 1;
                            false
                        }
                        _ => false,
                    };
                    !is_end
                } {}
                // 対応するOpenに移動したあとの状態
                // 次を読む
                self.next(cell)
            }
            // 対応するCloseの後まで移動するか、次を読むか
            Some(Operation::Open) => {
                // cellが0でない場合は次に進む
                if !cell.is_zero() {
                    return self.next(cell);
                };
                let mut open_count = 0;
                // 対応するCloseまで移動する
                while {
                    self.index += 1;
                    let next_op = self.program.0.get(self.index);
                    let is_end = match next_op {
                        // 本来は不正なbfプログラムなのだが、即座にtrueにするだけで許す
                        None => true,
                        // これがCloseならば対応してるCloseかどうかをチェック
                        Some(Operation::Close) => {
                            if open_count == 0 {
                                true
                            } else {
                                open_count -= 1;
                                false
                            }
                        }
                        // Openならまだ続きを読む
                        Some(Operation::Open) => {
                            open_count += 1;
                            false
                        }
                        _ => false,
                    };
                    !is_end
                } {}
                // 対応するCloseに移動したあとの状態
                // 次を読む
                self.next(cell)
            }
            Some(op) => Some(*op),
        }
    }
}

pub struct Interpreter {
    pub analyzer: Analyzer,
    pub memory: Memory,
}

impl Interpreter {
    fn next(&mut self, input: Option<ValueCell>) -> Option<ValueCell> {
        todo!()
    }
}
