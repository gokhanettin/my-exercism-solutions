#if !defined(CIRCULAR_BUFFER_H)
#define CIRCULAR_BUFFER_H

#include <vector>
#include <stdexcept>

namespace circular_buffer {
template <typename T>
class circular_buffer {
public:
    explicit circular_buffer(size_t size);
    T read();
    void write(const T& data);
    void write(T&& data);
    void overwrite(const T& data) noexcept;
    void overwrite(T&& data) noexcept;
    void clear() noexcept;
    bool empty() const noexcept;
    bool full() const noexcept;
    size_t size() const noexcept;

private:
    size_t front_;
    size_t rear_;
    size_t size_;
    std::vector<T> vec_;
};

template <typename T>
circular_buffer<T>::circular_buffer(size_t size)
    : front_{0}
    , rear_{0}
    , size_{0}
    , vec_(size)
{
}

template <typename T>
T circular_buffer<T>::read()
{
    if (empty()) {
        throw std::domain_error{"Cannot from empty buffer"};
    }
    T data = vec_[front_++];
    front_ = front_ % vec_.size();
    --size_;
    return data;
}

template <typename T>
void circular_buffer<T>::write(const T& data)
{
    if (full()) {
        throw std::domain_error{"Cannot write to full buffer"};
    }
    vec_[rear_++] = data;
    rear_ = rear_ % vec_.size();
    ++size_;
}

template <typename T>
void circular_buffer<T>::write(T&& data)
{
    if (full()) {
        throw std::domain_error{"Cannot write to full buffer"};
    }
    vec_[rear_++] = std::forward<T>(data);
    rear_ = rear_ % vec_.size();
    ++size_;
}

template <typename T>
void circular_buffer<T>::overwrite(const T& data) noexcept
{
    if (full()) {
        vec_[front_++] = data;
        front_ = front_ % vec_.size();
    } else {
        write(data);
    }
}

template <typename T>
void circular_buffer<T>::overwrite(T&& data) noexcept
{
    if (full()) {
        vec_[front_++] = std::forward<T>(data);
        front_ = front_ % vec_.size();
    } else {
        write(std::forward<T>(data));
    }
}

template <typename T>
inline void circular_buffer<T>::clear() noexcept
{
    size_ = 0;
    front_ = 0;
    rear_ = 0;
}

template <typename T>
inline bool circular_buffer<T>::empty() const noexcept
{
    return size_ == 0;
}

template <typename T>
inline bool circular_buffer<T>::full() const noexcept
{
    return size_ == vec_.size();
}

template <typename T>
inline size_t circular_buffer<T>::size() const noexcept
{
    return size_;
}

} // namespace circular_buffer

#endif // CIRCULAR_BUFFER_H