#include <linux/module.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/sched.h>
#include <asm/uaccess.h>
#include <linux/slab.h>
#include <linux/sort.h>

struct input_data
{
    char *input;
    size_t size;
};

static int result_part1 = 0;
static int result_part2 = 0;

static struct proc_dir_entry *parent = NULL;

static int next_int(const struct input_data *input_data, size_t *offset)
{
    int i = 0;

    // skip spaces
    while (*offset < input_data->size && isspace(input_data->input[*offset]))
        (*offset)++;

    // read int
    while (*offset < input_data->size && isdigit(input_data->input[*offset]))
    {
        i = i * 10 + input_data->input[*offset] - '0';
        (*offset)++;
    }

    return i;
}

static int int_cmp(const void *a, const void *b)
{
    return *(int *)a - *(int *)b;
}

static int int_abs(int x)
{
    return x < 0 ? -x : x;
}

static void solve(const struct input_data *input_data)
{
    size_t max_lines;
    int *left;
    int *right;
    size_t n;

    max_lines = 1; // 1 more if \n misses the last line
    for (size_t i = 0; i < input_data->size; ++i)
    {
        if (input_data->input[i] == '\n')
            max_lines += 1;
    }

    left = (int *)kmalloc(max_lines * sizeof(int), GFP_ATOMIC);
    right = (int *)kmalloc(max_lines * sizeof(int), GFP_ATOMIC);

    n = 0;
    size_t offset = 0;
    while (n < max_lines && offset < input_data->size)
    {
        left[n] = next_int(input_data, &offset);
        right[n] = next_int(input_data, &offset);

        n += 1;
    }

    sort(left, n, sizeof(int), int_cmp, NULL);
    sort(right, n, sizeof(int), int_cmp, NULL);

    // part 1
    result_part1 = 0;
    for (size_t i = 0; i < n; ++i)
    {
        result_part1 += int_abs(left[i] - right[i]);
    }

    // part 2
    result_part2 = 0;
    for (size_t i = 0; i < n; ++i)
    {
        int a = left[i];
        for (size_t j = 0; j < n; ++j)
        {
            if (right[j] == a)
            {
                result_part2 += a;
            }
        }
    }

    kfree(right);
    kfree(left);
}

static int result_proc_show(struct seq_file *m, void *v)
{
    if (m->private == NULL)
    {
        return 0;
    }

    if (strcmp("part1", (const char *)m->private) == 0)
    {
        seq_printf(m, "%d\n", result_part1);
    }
    else if (strcmp("part2", (const char *)m->private) == 0)
    {
        seq_printf(m, "%d\n", result_part2);
    }

    return 0;
}

static int result_proc_open(struct inode *inode, struct file *file)
{
    if (file == NULL || file->f_path.dentry == NULL)
    {
        return -1;
    }

    return single_open(file, result_proc_show, (void *)file->f_path.dentry->d_name.name);
}

static const struct proc_ops result_fops = {
    .proc_open = result_proc_open,
    .proc_read = seq_read,
    .proc_lseek = seq_lseek,
    .proc_release = single_release,
};

/*
 * /proc/aoc/2024/day/input pseudo file
 */
static int input_open_proc(struct inode *inode, struct file *file)
{
    pr_info("input file opened\n");

    struct input_data *input_data = (struct input_data *)kmalloc(sizeof(struct input_data), GFP_ATOMIC);

    input_data->input = NULL;
    input_data->size = 0;

    file->private_data = (void *)input_data;

    return 0;
}

static int input_release_proc(struct inode *inode, struct file *file)
{
    pr_info("input file released.....\n");

    struct input_data *input_data = (struct input_data *)file->private_data;

    solve(input_data);

    if (input_data != NULL)
    {
        kfree(input_data->input);
        kfree(input_data);
    }
    return 0;
}

static ssize_t input_write_proc(struct file *file, const char *buff, size_t len, loff_t *off)
{
    if (access_ok(buff, len) == 0)
    {
        pr_err("Read input error.\n");
        return 0;
    }

    struct input_data *input_data = (struct input_data *)file->private_data;

    input_data->input = (char *)kmalloc(len, GFP_ATOMIC);
    input_data->size = len;

    unsigned int n = copy_from_user(input_data->input, buff, len);

    if (n != 0)
    {
        pr_err("Read input error. ret=%u\n", n);
    }

    return len;
}

static struct proc_ops input_fops = {
    .proc_open = input_open_proc,
    .proc_write = input_write_proc,
    .proc_release = input_release_proc,
};

static int __init aoc_2024_init(void)
{
    parent = proc_mkdir("aoc", NULL);

    if (parent == NULL)
    {
        pr_info("Cannot create aoc filesystem");
        return -1;
    }

    proc_create("input", 0222, parent, &input_fops);
    proc_create("part1", 0444, parent, &result_fops);
    proc_create("part2", 0444, parent, &result_fops);

    return 0;
}

static void __exit aoc_2024_exit(void)
{
    if (parent != NULL)
    {
        remove_proc_entry("input", parent);
        remove_proc_entry("part1", parent);
        remove_proc_entry("part2", parent);
        proc_remove(parent);
    }
    parent = NULL;
}

module_init(aoc_2024_init);
module_exit(aoc_2024_exit);

MODULE_LICENSE("GPL");
MODULE_AUTHOR("rene-d");
MODULE_DESCRIPTION("Advent of Code 2024");
MODULE_VERSION("0.1");
