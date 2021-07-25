#if !defined(BINARY_SEARCH_TREE_H)
#define BINARY_SEARCH_TREE_H

#include <memory>
#include <vector>

namespace binary_search_tree {

template <typename T>
class binary_tree {
public:
    class iterator {
    public:
        using iterator_category = std::forward_iterator_tag;
        using value_type = T;
        using reference = const T&;
        using pointer = const T*;
        using difference_type = std::ptrdiff_t;

        reference operator*() const;
        pointer operator->() const;
        iterator& operator++();
        iterator operator++(int);
        friend bool operator==(const iterator& lhs, const iterator& rhs)
        { return lhs.pos_ == rhs.pos_; }
        friend bool operator!=(const iterator& lhs, const iterator& rhs)
        {return !(lhs == rhs); }
    private:
        iterator(const binary_tree* tree, std::size_t pos);

        const binary_tree* tree_;
        std::size_t pos_;

        friend class binary_tree;
    };

    using pointer_type = std::unique_ptr<binary_tree>;

    explicit binary_tree(const T& value);
    const T& data() const;
    const pointer_type& left() const;
    const pointer_type& right() const;
    void insert(const T& value);
    void insert(T&& value);
    const T& at(std::size_t pos) const;
    std::size_t size() const;

    iterator begin() const;
    iterator end() const;

private:
    T value_;
    std::size_t size_;
    pointer_type left_;
    pointer_type right_;
};

template <typename T>
binary_tree<T>::binary_tree(const T& value)
    : value_{value}, size_{1}, left_{nullptr}, right_{nullptr}
{
}

template <typename T>
const T& binary_tree<T>::data() const
{
    return value_;
}

template <typename T>
const typename binary_tree<T>::pointer_type&
binary_tree<T>::left() const
{
    return left_;
}

template <typename T>
const typename binary_tree<T>::pointer_type&
binary_tree<T>::right() const
{
    return right_;
}

template <typename T>
void binary_tree<T>::insert(const T& value)
{
    auto tree = this;
    while (true) {
        ++tree->size_;
        auto& child = value > tree->value_ ? tree->right_ : tree->left_;
        if (!child) {
            child = std::make_unique<binary_tree>(value);
            break;
        }
        tree = child.get();
    }
}

template <typename T>
void binary_tree<T>::insert(T&& value)
{
    auto tree = this;
    while (true) {
        ++tree->size_;
        auto& child = value > tree->value_ ? tree->right_ : tree->left_;
        if (!child) {
            child = std::make_unique<binary_tree>(std::move(value));
            break;
        }
        tree = child.get();
    }
}

template <typename T>
const T& binary_tree<T>::at(std::size_t pos) const
{
    auto tree = this;
    while (true) {
        auto left_size = tree->left_ ? tree->left_->size() : 0;
        if (pos < left_size) {
            tree = tree->left_.get();
        } else if (pos > left_size) {
            tree = tree->right_.get();
            pos -= (left_size + 1);
        } else {
            break;
        }
    }
    return tree->data();
}

template <typename T>
std::size_t binary_tree<T>::size() const
{
    return size_;
}

template <typename T>
typename binary_tree<T>::iterator
binary_tree<T>::begin() const
{
    return iterator{this, 0};
}

template <typename T>
typename binary_tree<T>::iterator
binary_tree<T>::end() const
{
    return iterator{this, size_};
}

template <typename T>
binary_tree<T>::iterator::iterator(const binary_tree* tree, std::size_t pos)
    : tree_{tree}, pos_{pos}
{
}

template <typename T>
const T& binary_tree<T>::iterator::operator*() const
{
    return tree_->at(pos_);
}

template <typename T>
const T* binary_tree<T>::iterator::operator->() const
{
    return &**this;
}

template <typename T>
typename binary_tree<T>::iterator&
binary_tree<T>::iterator::operator++()
{
    ++pos_;
    return *this;
}

template <typename T>
typename binary_tree<T>::iterator
binary_tree<T>::iterator::operator++(int)
{
    iterator ret{*this};
    ++*this;
    return ret;
}
} // namespace binary_search_tree

#endif // BINARY_SEARCH_TREE_H