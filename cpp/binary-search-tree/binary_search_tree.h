#if !defined(BINARY_SEARCH_TREE_H)
#define BINARY_SEARCH_TREE_H

#include <memory>
#include <stack>

namespace binary_search_tree {

template <typename T>
class binary_tree;

namespace {
    template <typename T>
    class binary_tree_iterator;
}

template <typename T>
class binary_tree {
public:
    using iterator = binary_tree_iterator<T>;

    explicit binary_tree(const T& val);
    const T& data() const;
    const std::unique_ptr<binary_tree<T>>& left() const;
    const std::unique_ptr<binary_tree<T>>& right() const;
    void insert(const T& val);

    iterator begin() const;
    iterator end() const;

private:
    T data_;
    std::unique_ptr<binary_tree<T>> left_;
    std::unique_ptr<binary_tree<T>> right_;
    void do_insert(std::unique_ptr<binary_tree<T>>& tree, const T& val);
};

namespace {
    template <typename T>
    class binary_tree_iterator {
    public:
        explicit binary_tree_iterator(const binary_tree<T>* tree = nullptr)
        {
            add_tree(tree);
        }

        const T& operator*() const
        {
            return stack_.top()->data();
        }
        binary_tree_iterator& operator++()
        {
            if (stack_.empty()) {
                return *this;
            }
            auto tree = stack_.top();
            stack_.pop();
            if (tree->right()) {
                add_tree(tree->right().get());
            }
            return *this;
        }
        binary_tree_iterator operator++(int)
        {
            binary_tree_iterator tmp{*this};
            ++(*this);
            return tmp;
        }
        friend bool operator==(const binary_tree_iterator& lhs,
            const binary_tree_iterator& rhs)
        {
            return lhs.stack_ == rhs.stack_;
        }
        friend bool operator!=(const binary_tree_iterator& lhs,
            const binary_tree_iterator& rhs)
        {
            return !(lhs == rhs);
        }

    private:
        std::stack<const binary_tree<T>*> stack_;

        void add_tree(const binary_tree<T>* tree)
        {
            while (tree) {
                stack_.push(tree);
                tree = tree->left().get();
            }
        }
    };
}

template <typename T>
binary_tree<T>::binary_tree(const T& val)
    : data_{val}
    , left_{nullptr}
    , right_{nullptr}
{
}

template <typename T>
const T& binary_tree<T>::data() const
{
    return data_;
}

template <typename T>
const std::unique_ptr<binary_tree<T>>& binary_tree<T>::left() const
{
    return left_;
}

template <typename T>
const std::unique_ptr<binary_tree<T>>& binary_tree<T>::right() const
{
    return right_;
}

template <typename T>
void binary_tree<T>::insert(const T& val)
{
    do_insert(val > data_ ? right_ : left_, val);
}

template <typename T>
typename binary_tree<T>::iterator binary_tree<T>::begin() const
{
    return iterator{this};
}

template <typename T>
typename binary_tree<T>::iterator binary_tree<T>::end() const
{
    return iterator{nullptr};
}

template <typename T>
void binary_tree<T>::do_insert(std::unique_ptr<binary_tree<T>>& tree,
    const T& val)
{
    if (!tree) {
        tree = std::make_unique<binary_tree<T>>(val);
    } else {
        do_insert(val > tree->data_ ? tree->right_ : tree->left_, val);
    }
}

} // namespace binary_search_tree

#endif // BINARY_SEARCH_TREE_H