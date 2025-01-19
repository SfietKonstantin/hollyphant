#pragma once

#include <memory>
#include <rust/cxx.h>

class RustEventPublisherImpl;
class RustEventPublisher
{
public:
    explicit RustEventPublisher(std::unique_ptr<RustEventPublisherImpl> impl);
    ~RustEventPublisher();
    void publish_set(const std::string &key, const std::string &value) const;

private:
    std::unique_ptr<RustEventPublisherImpl> m_impl;
};
