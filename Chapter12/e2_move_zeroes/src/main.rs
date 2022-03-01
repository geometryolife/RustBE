// 【零移动】

// 【题目描述】
// 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，
// 同时保持非零元素的相对顺序。

// 【说明】
// （1）必须在原数组上操作，不能复制额外的数组。
// （2）尽量减少操作次数。

// 【示例】
// 输入：nums = [0,1,0,3,12]
// 输出：[1,3,12,0,0]
// 输入：nums = [0]
// 输出：[0]

// 【链接】https://leetcode-cn.com/problems/move-zeroes

// 【解题思路】
// 遍历数组 nums，把非零元素（假设有 i 个）按顺序存入数组第 0～i-1 个位置，
// 再把数组的第 i～n-i 个位置全部设为 0。

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;

        // 将非零元素按顺序存入数组的 0 至 i-1 位置
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
        }

        // 把剩余部分全部设置为 0
        for k in i..nums.len() {
            nums[k] = 0;
        }
    }
}

fn main() {
    let mut vec: Vec<i32> = vec![0, 1, 0, 3, 12];
    println!("{:?}", vec);

    Solution::move_zeroes(&mut vec);
    println!("{:?}", vec);
}
