// 【加一】

// 【题目描述】
// 给定一个由整数组成的非空数组所表示的非负整数，在该数的基础上加一。
// 最高位数字放在数组的首位，数组中每个元素只存储单个数字。

// 【说明】
// 假设除了整数 0 之外，这个整数不会以零开头。

// 【示例】
// 输入：[1,2,3]
// 输出：[1,2,4]
// 输入：[9,9,9]
// 输出：[1,0,0,0]

// 【链接】https://leetcode-cn.com/problems/plus-one

// 【解题思路】
// 反向遍历数组 digits，依次取元素加一，可能出现以下 3 种情况。
// （1）末位数字非 9：末位加一，前面所有数字保持不变。
// （2）末位数字非 9，但非全部数字都是 9：末位加一后，到位置 i 进位停止，
// digits[i] 加一，digits[i+1] 到 digits[digits.len()-1] 全部置为 0。
// （3）全部数字都是 9：新建一个数组 nums，长度为 digits.len()+1，
// 除了 nums[0] 为 1 外，其余元素皆为 0。

struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut i = digits.len() - 1;

        loop {
            // 数字非 9，直接加一返回
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }

            // 数字是 9，将其置为 0
            digits[i] = 0;
            if i > 0 {
                i -= 1;
            } else if i == 0 {
                // 全部数字是 9，跳出循环
                break;
            }
        }

        // 重置数组，数组长度因进位而加一，除第一个元素为 1 外，其余元素皆为 0
        digits = vec![0; digits.len() + 1];
        digits[0] = 1;
        return digits;
    }
}

fn main() {
    let vec: Vec<i32> = vec![1,2,3];
    println!("{:?}", vec);

    let vec = Solution::plus_one(vec);
    println!("{:?}", vec);

    let vec: Vec<i32> = vec![9,9,9];
    println!("{:?}", vec);

    let vec = Solution::plus_one(vec);
    println!("{:?}", vec);
}

// === Output ===
// [1, 2, 3]
// [1, 2, 4]
// [9, 9, 9]
// [1, 0, 0, 0]
