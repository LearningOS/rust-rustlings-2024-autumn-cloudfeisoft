use std::collections::LinkedList;

pub struct MergeableLinkedList<T> {
    list: LinkedList<T>,
}

impl<T: Ord + Clone> MergeableLinkedList<T> {
    // 创建一个新的 MergeableLinkedList
    pub fn new(list: LinkedList<T>) -> Self {
        MergeableLinkedList { list }
    }

    // 合并两个有序的 MergeableLinkedList
    pub fn merge(mut self, other: MergeableLinkedList<T>) -> Self {
        let mut result = LinkedList::new();

        // 使用两个迭代器来遍历两个链表
        let mut iter_a = self.list.iter();
        let mut iter_b = other.list.iter();

        // 使用 `match` 语句来比较两个链表的当前元素
        while let (Some(val_a), Some(val_b)) = (iter_a.next(), iter_b.next()) {
            if val_a < val_b {
                // 如果 val_a 的值小于 val_b 的值，将 val_a 添加到结果链表
                result.push_back(val_a.clone());
            } else {
                // 否则，将 val_b 添加到结果链表
                result.push_back(val_b.clone());
            }
        }

        // 将剩余的元素添加到结果链表
        result.append(&mut self.list);
        result.extend(other.list);

        // 返回包装了合并后链表的新实例
        MergeableLinkedList { list: result }
    }
}

fn main() {
    let mut list_a: LinkedList<i32> = LinkedList::new();
    let mut list_b: LinkedList<i32> = LinkedList::new();

    // 假设 list_a 和 list_b 已经被正确填充了有序元素
    list_a.push_back(1);
    list_a.push_back(3);
    list_a.push_back(5);

    list_b.push_back(2);
    list_b.push_back(4);
    list_b.push_back(6);

    let merged_list = MergeableLinkedList::new(list_a).merge(MergeableLinkedList::new(list_b));

    // 打印合并后的链表
    for elem in merged_list.list.iter() {
        println!("{}", elem);
    }
}