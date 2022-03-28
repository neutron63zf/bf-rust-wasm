// メモリセル
// 一般的bf
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ValueCell(u8);

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
        todo!()
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
        todo!()
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
