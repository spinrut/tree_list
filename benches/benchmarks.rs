#![feature(test)]

extern crate test;
use test::Bencher;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use tree_list::recursive_tree_list::RecursiveTreeList;
use tree_list::tree_list::TreeList;

#[bench]
fn tree_insert_delete_random_1000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut tree: TreeList<char> = TreeList::new();

      for i in 0..1000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         tree.insert(ind, c);
      }

      for i in (0..500).rev() {
         let ind = rng.gen_range(0, i + 1);
         tree.remove(ind);
      }

      tree.clear();
   });
}

#[bench]
fn rec_tree_insert_delete_random_1000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

      for i in 0..1000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         tree.insert(ind, c);
      }

      for i in (0..500).rev() {
         let ind = rng.gen_range(0, i + 1);
         tree.remove(ind);
      }

      tree.clear();
   });
}

#[bench]
fn vec_insert_delete_random_1000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut vec: Vec<char> = Vec::new();

      for i in 0..1000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         vec.insert(ind, c);
      }

      for i in (0..500).rev() {
         let ind = rng.gen_range(0, i + 1);
         vec.remove(ind);
      }

      vec.clear();
   });
}

#[bench]
fn tree_insert_delete_random_10000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut tree: TreeList<char> = TreeList::new();

      for i in 0..10000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         tree.insert(ind, c);
      }

      for i in (0..5000).rev() {
         let ind = rng.gen_range(0, i + 1);
         tree.remove(ind);
      }

      tree.clear();
   });
}

#[bench]
fn rec_tree_insert_delete_random_10000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

      for i in 0..10000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         tree.insert(ind, c);
      }

      for i in (0..5000).rev() {
         let ind = rng.gen_range(0, i + 1);
         tree.remove(ind);
      }

      tree.clear();
   });
}

#[bench]
fn vec_insert_delete_random_10000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut vec: Vec<char> = Vec::new();

      for i in 0..10000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         vec.insert(ind, c);
      }

      for i in (0..5000).rev() {
         let ind = rng.gen_range(0, i + 1);
         vec.remove(ind);
      }

      vec.clear();
   });
}

#[bench]
fn tree_insert_delete_random_20000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut tree: TreeList<char> = TreeList::new();

      for i in 0..20000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         tree.insert(ind, c);
      }

      for i in (0..10000).rev() {
         let ind = rng.gen_range(0, i + 1);
         tree.remove(ind);
      }

      tree.clear();
   });
}

#[bench]
fn rec_tree_insert_delete_random_20000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

      for i in 0..20000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         tree.insert(ind, c);
      }

      for i in (0..10000).rev() {
         let ind = rng.gen_range(0, i + 1);
         tree.remove(ind);
      }

      tree.clear();
   });
}

#[bench]
fn vec_insert_delete_random_20000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut vec: Vec<char> = Vec::new();

      for i in 0..20000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         vec.insert(ind, c);
      }

      for i in (0..10000).rev() {
         let ind = rng.gen_range(0, i + 1);
         vec.remove(ind);
      }

      vec.clear();
   });
}

#[bench]
fn tree_insert_delete_random_30000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut tree: TreeList<char> = TreeList::new();

      for i in 0..30000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         tree.insert(ind, c);
      }

      for i in (0..15000).rev() {
         let ind = rng.gen_range(0, i + 1);
         tree.remove(ind);
      }

      tree.clear();
   });
}

#[bench]
fn rec_tree_insert_delete_random_30000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

      for i in 0..30000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         tree.insert(ind, c);
      }

      for i in (0..15000).rev() {
         let ind = rng.gen_range(0, i + 1);
         tree.remove(ind);
      }

      tree.clear();
   });
}

#[bench]
fn vec_insert_delete_random_30000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut vec: Vec<char> = Vec::new();

      for i in 0..30000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         vec.insert(ind, c);
      }

      for i in (0..15000).rev() {
         let ind = rng.gen_range(0, i + 1);
         vec.remove(ind);
      }

      vec.clear();
   });
}

#[bench]
fn tree_insert_delete_random_40000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut tree: TreeList<char> = TreeList::new();

      for i in 0..40000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         tree.insert(ind, c);
      }

      for i in (0..20000).rev() {
         let ind = rng.gen_range(0, i + 1);
         tree.remove(ind);
      }

      tree.clear();
   });
}

#[bench]
fn rec_tree_insert_delete_random_40000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

      for i in 0..40000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         tree.insert(ind, c);
      }

      for i in (0..20000).rev() {
         let ind = rng.gen_range(0, i + 1);
         tree.remove(ind);
      }

      tree.clear();
   });
}

#[bench]
fn vec_insert_delete_random_40000(bencher: &mut Bencher) {
   let mut rng = StdRng::seed_from_u64(0);

   bencher.iter(|| {
      let mut vec: Vec<char> = Vec::new();

      for i in 0..40000 {
         let ind = rng.gen_range(0, i + 1);
         let c = rng.gen_range('a' as u8, 'z' as u8 + 1) as char;
         vec.insert(ind, c);
      }

      for i in (0..20000).rev() {
         let ind = rng.gen_range(0, i + 1);
         vec.remove(ind);
      }

      vec.clear();
   });
}