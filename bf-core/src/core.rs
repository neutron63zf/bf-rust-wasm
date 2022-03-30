// メモリセル
// 一般的bf
#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct ValueCell(u8);

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
struct Memory {
    // ポインタの位置を数字で持ちたくないのでこうなる
    before: Vec<ValueCell>, // ポインタの前のメモリセル
    after: Vec<ValueCell>,  // ポインタの後のメモリセル
    current: ValueCell,     // 現在のポインタの指すメモリセル
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
enum Operation {
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
struct Program {
    // プログラムでもまた、indexを持たずに現在のOperationを持つようにする
    before: Vec<Operation>,
    current: Option<Operation>,
    after: Vec<Operation>,
}

enum ProgramPointerMove {
    Next,
    Prev,
}

impl Program {
    fn from_operations(operations: Vec<Operation>) -> Self {
        if let Some((first, operations)) = operations.split_first() {
            let mut operations = operations.to_vec();
            operations.reverse();
            return Self {
                before: vec![],
                current: Some(*first),
                after: operations,
            };
        }
        Self {
            before: vec![],
            current: None,
            after: vec![],
        }
    }
    fn move_pointer(&mut self, instruction: ProgramPointerMove) {
        match instruction {
            ProgramPointerMove::Next => {
                if self.current.is_some() {
                    self.before.push(self.current.unwrap());
                }
                self.current = self.after.pop();
            }
            ProgramPointerMove::Prev => {
                if self.current.is_some() {
                    self.after.push(self.current.unwrap());
                }
                self.current = self.before.pop();
            }
        }
    }
}

// 本当はOpen, Closeの命令に対応するような入れ子構造になったほうやつもあったほうがいい気がするがとりあえずこれで

enum AnalyzerPointerJump {
    ToOpen,
    ToClose,
}

// 現在のポインタの指す値が与えられたら次の命令は計算できるのでこれで十分
struct Analyzer {
    program: Program,
    pair_count: Vec<()>,
    jump_type: Option<AnalyzerPointerJump>,
}

impl Analyzer {
    fn initialize(program: Program) -> Self {
        Self {
            program,
            pair_count: vec![],
            jump_type: None,
        }
    }
    fn encountered_target_operation(&mut self) {
        if self.pair_count.is_empty() {
            self.jump_type = None;
        } else {
            self.pair_count.pop();
        }
    }
    fn encountered_opposite_operation(&mut self) {
        self.pair_count.push(());
    }
    // 次の命令を計算するが、ジャンプ中はNoneを返す
    fn next(&mut self, cell: ValueCell) -> Option<Operation> {
        // openに向かって進んでいるときはポインタは左に動かす
        let next_move = match self.jump_type {
            Some(AnalyzerPointerJump::ToOpen) => ProgramPointerMove::Prev,
            _ => ProgramPointerMove::Next,
        };
        // ポインタを動かして、命令を読む
        self.program.move_pointer(next_move);
        let next_operation = self.program.current;
        let current_jump_type = &self.jump_type;
        // 命令に応じて移動状態を変化させる
        match (next_operation, current_jump_type) {
            // 求めているものに遭遇したとき
            (Some(Operation::Open), Some(AnalyzerPointerJump::ToOpen)) => {
                self.encountered_target_operation();
                None
            }
            (Some(Operation::Close), Some(AnalyzerPointerJump::ToClose)) => {
                self.encountered_target_operation();
                None
            }
            // 求めているものの逆に遭遇したとき
            (Some(Operation::Close), Some(AnalyzerPointerJump::ToOpen)) => {
                self.encountered_opposite_operation();
                None
            }
            (Some(Operation::Open), Some(AnalyzerPointerJump::ToClose)) => {
                self.encountered_opposite_operation();
                None
            }
            // 普通にプログラムを実行していてOpen, Closeに遭遇したとき
            (Some(Operation::Open), None) => {
                // セルが0のときだけCloseに向かって飛ぶ
                if cell.is_zero() {
                    self.jump_type = Some(AnalyzerPointerJump::ToClose);
                }
                None
            }
            (Some(Operation::Close), None) => {
                // セルが0でないときだけOpenに向かって飛ぶ
                if !cell.is_zero() {
                    self.jump_type = Some(AnalyzerPointerJump::ToOpen);
                }
                None
            }
            // 移動中だったら命令はなし
            (_, Some(_)) => None,
            // 移動中でないときに普通の命令に遭遇したらそれを返す
            (_, None) => next_operation,
        }
    }
}

struct Interpreter {
    analyzer: Analyzer,
    memory: Memory,
}

impl Interpreter {
    fn next(&mut self, input: Option<ValueCell>) -> Option<ValueCell> {
        todo!()
    }
}
