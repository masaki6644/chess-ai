use std::vec::Vec;

pub type Hash = u64;

// =========================
// 置換表エントリ
// =========================
#[derive(Clone, Copy, Debug)]
pub struct TTEntry {
    pub hash: Hash,
    pub depth: u8,
    pub score: f32,
    pub flag: TTFlag,
    pub best_move: u16, // とりあえず簡易（後でMove型に差し替え）
}

// =========================
// スコアの種類
// =========================
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TTFlag {
    Exact,
    Alpha, // fail-low
    Beta,  // fail-high
}

// =========================
// Transposition Table本体
// =========================
pub struct TranspositionTable {
    table: Vec<Option<TTEntry>>,
    mask: usize,
}

impl TranspositionTable {
    // size_mb: 例 64MB
    pub fn new(size_mb: usize) -> Self {
        let entry_size = std::mem::size_of::<Option<TTEntry>>();

        let size = (size_mb * 1024 * 1024) / entry_size;
        let size = size.next_power_of_two();

        Self {
            table: vec![None; size],
            mask: size - 1,
        }
    }

    #[inline]
    fn index(&self, hash: Hash) -> usize {
        (hash as usize) & self.mask
    }

    // =========================
    // 探索（読み取り）
    // =========================
    pub fn probe(&self, hash: Hash, depth: u8) -> Option<TTEntry> {
        let idx = self.index(hash);

        match self.table[idx] {
            Some(e) if e.hash == hash && e.depth >= depth => Some(e),
            _ => None,
        }
    }

    // =========================
    // 保存（書き込み）
    // =========================
    pub fn store(&mut self, entry: TTEntry) {
        let idx = self.index(entry.hash);

        match self.table[idx] {
            Some(old) => {
                // 深い探索を優先
                if entry.depth >= old.depth {
                    self.table[idx] = Some(entry);
                }
            }
            None => {
                self.table[idx] = Some(entry);
            }
        }
    }

    // =========================
    // クリア
    // =========================
    pub fn clear(&mut self) {
        self.table.fill(None);
    }

    // =========================
    // 統計（デバッグ用）
    // =========================
    pub fn usage(&self) -> f32 {
        let used = self.table.iter().filter(|e| e.is_some()).count();
        used as f32 / self.table.len() as f32
    }
}