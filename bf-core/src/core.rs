// メモリセル
// 一般的bf
pub struct ValueCell(u8);

// データメモリ
pub struct Memory {
    // ポインタの位置を数字で持ちたくないのでこうなる
    pub before: Vec<ValueCell>, // ポインタの前のメモリセル
    pub after: Vec<ValueCell>,  // ポインタの後のメモリセル
    pub current: ValueCell,     // 現在のポインタの指すメモリセル
}

// 命令セット
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

pub struct Program(Vec<Operation>);
