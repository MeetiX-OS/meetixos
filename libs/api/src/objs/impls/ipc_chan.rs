/*! # Interprocess Communication Channel
 *
 * Implements the IPC channel to communicate via messages with other tasks
 */

use core::num::NonZeroUsize;

use os::sysc::{
    codes::KernIpcChanFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::obj::{
        ObjType,
        RecvMode
    },
    caller::{
        KernCaller,
        Result
    },
    objs::{
        ObjId,
        Object,
        SizeableData,
        UserCreatable
    }
};

/** # Inter Process Communication Channel
 *
 * Represents a reference to an open communication channel.
 *
 * With this object it is possible to send and/or receive arbitrary sized
 * binary messages between different processes/threads.
 */
#[repr(transparent)]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IpcChan {
    m_handle: ObjId
}

impl IpcChan {
    /** # Sends a new message
     *
     * The message can have arbitrary size but must implement the [`AsRef`]
     * trait, to be able to tell to the kernel which is his size in bytes
     * and treat it like an [`u8 slice`].
     *
     * The `id` is the transaction identifier, used to direct the message to
     * a particular receiver; when [`None`] is given the message is
     * broadcast to everyone that receive messages without transaction
     * id.
     *
     * When the transaction identifier (`id`) not already exists it is
     * automatically created by the kernel, otherwise, if it already exists,
     * the message is appended to the queue which have the same id
     *
     * [`AsRef`]: core::convert::AsRef
     * [`u8 slice`]: https://doc.rust-lang.org/std/primitive.slice.html
     * [`None`]: core::option::Option::None
     */
    pub fn send_msg<T>(&self, msg: &T, id: Option<NonZeroUsize>) -> Result<usize>
        where T: AsRef<[u8]> {
        self.kern_call_3(KernFnPath::IpcChan(KernIpcChanFnId::Send),
                         msg.as_ref().as_ptr() as usize,
                         msg.as_ref().len(),
                         id.map(|value| value.get()).unwrap_or(0))
    }

    /** # Receives the last message
     *
     * The `msg` is filled with the content of the received message.
     *
     * If no messages that are no more than `msg.as_mut().len()` exist an
     * error is returned.
     *
     * The `id` is the transaction identifier, used to receive the message
     * from a particular sender; when [`None`] is given the message is
     * received from anyone that send messages without transaction id.
     *
     * The transaction identifier (`id`) must already exist or an error is
     * returned.
     *
     * The system call returns the size of the message received in bytes
     * when [`Ok`]
     *
     * [`None`]: core::option::Option::None
     * [`Ok`]: core::result::Result::Ok
     */
    pub fn recv_msg<T>(&self,
                       mode: RecvMode,
                       msg: &mut T,
                       id: Option<NonZeroUsize>)
                       -> Result<usize>
        where T: AsMut<[u8]> {
        self.kern_call_4(KernFnPath::IpcChan(KernIpcChanFnId::Recv),
                         mode.into(),
                         msg.as_mut().as_mut_ptr() as usize,
                         msg.as_mut().len(),
                         id.map(|value| value.get()).unwrap_or(0))
    }
}

impl Object for IpcChan {
    /** The value of the [`ObjType`] that matches the implementation
     *
     * [`ObjType`]: crate::bits::obj::types::ObjType
     */
    const TYPE: ObjType = ObjType::IpcChan;

    /** Returns the immutable reference to the underling [`ObjId`] instance
     *
     * [`ObjId`]: crate::objs::ObjId
     */
    fn obj_handle(&self) -> &ObjId {
        &self.m_handle
    }

    /** Returns the mutable reference to the underling [`ObjId`] instance
     *
     * [`ObjId`]: crate::objs::ObjId
     */
    fn obj_handle_mut(&mut self) -> &mut ObjId {
        &mut self.m_handle
    }
}

impl From<ObjId> for IpcChan {
    /** Performs the conversion
     */
    fn from(id: ObjId) -> Self {
        Self { m_handle: id }
    }
}

impl KernCaller for IpcChan {
    /** Returns the upper 32bits of the 64bit identifier of a system call
     */
    fn caller_handle_bits(&self) -> u32 {
        self.obj_handle().caller_handle_bits()
    }
}

impl SizeableData for IpcChan {
    /* No methods to implement */
}

impl UserCreatable for IpcChan {
    /* No methods to implement */
}
